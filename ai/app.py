import pickle
import insightface
import cv2
import numpy as np
from PIL import Image
from io import BytesIO
from fastapi import FastAPI, UploadFile, Response

app = FastAPI()

model = pickle.load(open("lr-if-alv3.sav", "rb"))

faceapp = insightface.app.FaceAnalysis(
    name="antelopev2", providers=["CPUExecutionProvider"]
)
faceapp.prepare(ctx_id=0)


@app.post("/", responses={200: {"content": {"image/png": {}}}}, response_class=Response)
async def radar(file: UploadFile):
    contents = await file.read()
    img = Image.open(BytesIO(contents))
    img = np.array(img.convert("RGB"))

    faces = faceapp.get(img)

    if faces:
        faces_embs = [f.embedding for f in faces]
        faces_probas = [r[1] for r in model.predict_proba(faces_embs)]

        for face, proba in zip(faces, faces_probas):
            box = face.bbox.astype(int)
            if proba >= 0.5:
                box_color = (255, 102, 255)
            else:
                box_color = (51, 204, 51)
            cv2.rectangle(img, (box[0], box[1]), (box[2], box[3]), box_color, 2)

            for dot in face.landmark_3d_68:
                cv2.circle(img, (int(dot[0]), int(dot[1])), 1, (0, 255, 255), 2)

            res = f"{int(proba * 100)}%"

            cv2.putText(
                img,
                res,
                (box[0] - 2, box[1] - 8),
                cv2.FONT_HERSHEY_COMPLEX,
                1.25,
                box_color,
                2,
            )

    cv2.putText(
        img,
        "@gayradarbot",
        (5, img.shape[0] - 16),
        cv2.FONT_HERSHEY_COMPLEX,
        1.5,
        (255, 0, 0),
        2,
    )

    img = Image.fromarray(img)
    img_io = BytesIO()
    img.save(img_io, "PNG")
    img_io.seek(0)
    img_b = img_io.read()

    return Response(content=img_b, media_type="image/png")
