# Gay Radar

Telegram bot: [@gayradarbot](https://t.me/gayradarbot)

Determining the sexual orientation of men from photos.

Pipeline: image -> [insightface](https://insightface.ai/) (embeddings) -> logistic regression.

Training data: ~12k photos of Russian men of European appearance from 16 to 60 years old from the [Mamba](https://www.mamba.ru/) dating website.

Accuracy: ~60% overall, up to 80% when model's result > 0.8.
