use std::sync::Arc;

use anyhow::Result;
use reqwest::Client;
use sqlx::PgPool;
use teloxide::{
    adaptors::{throttle::Limits, Throttle},
    dispatching::{Dispatcher, UpdateFilterExt},
    dptree,
    net::Download,
    requests::{Requester, RequesterExt},
    types::{InputFile, Message, Update},
};
use tokio::sync::Mutex;

type Bot = Throttle<teloxide::Bot>;

mod db;

#[derive(Default)]
struct Ai {
    client: Client,
    lock: Mutex<()>,
}

impl Ai {
    async fn process_image(&self, img: Vec<u8>) -> Result<Vec<u8>> {
        let lock = self.lock.lock().await;

        let file_part = reqwest::multipart::Part::bytes(img).file_name("file");
        let form = reqwest::multipart::Form::new().part("file", file_part);
        let req = self
            .client
            .post("http://127.0.0.1:8804/")
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;
        drop(lock);

        Ok(req.bytes().await?.to_vec())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let db = sqlx::PgPool::connect(&std::env::var("DATABASE_URL")?).await?;
    sqlx::migrate!().run(&db).await?;

    let bot = teloxide::Bot::from_env().throttle(Limits::default());

    let handler = Update::filter_message().branch(dptree::endpoint(answer));

    let ai = Arc::new(Ai::default());

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![db, ai])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}

async fn answer(db: PgPool, bot: Bot, msg: Message, ai: Arc<Ai>) -> Result<()> {
    if let Err(e) = try_answer(&db, &bot, &msg, &ai).await {
        bot.send_message(
            msg.chat.id,
            format!("Произошла ошибка: {e}, попробуйте позднее или обратитесь к @averyanalex"),
        )
        .await?;
    };

    Ok(())
}

async fn try_answer(db: &PgPool, bot: &Bot, msg: &Message, ai: &Ai) -> Result<()> {
    db::update_user(db, msg.chat.id.0).await?;

    match msg.photo() {
        Some([.., photo]) => {
            let wait_msg = bot
                .send_message(msg.chat.id, "Идёт обработка, подождите немного...")
                .await?;

            let file = bot.get_file(&photo.file.id).await?;
            let mut dst = Vec::new();
            bot.download_file(&file.path, &mut dst).await?;

            let res = ai.process_image(dst).await?;

            let photo = InputFile::memory(res);
            bot.send_photo(msg.chat.id, photo).await?;
            bot.delete_message(wait_msg.chat.id, wait_msg.id).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Просто отправьте фото человека, и бот вышлет шанс того, \
            что перед вами гей. К примеру, если шанс 50% - бот не знает, кто это, но если шанс 80% или 20% - перед \
            вами с высокой точностью гей или натурал соответственно.\n\nНейросеть обучена на российских мужчинах \
            европейской внешности от 18 до 60 лет, не используйте бота на \
            подростках, женщинах и лицах других национальностей.").await?;
        }
    };
    Ok(())
}
