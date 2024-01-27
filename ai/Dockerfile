FROM python:3.11 AS builder
RUN python -m pip install --no-cache-dir poetry==1.7
COPY poetry.lock pyproject.toml ./
RUN poetry export --without-hashes --without dev -f requirements.txt -o requirements.txt

FROM python:3.11 as unzip
WORKDIR /unzip
COPY ./antelopev2.zip /unzip
RUN unzip antelopev2.zip

FROM python:3.11
WORKDIR /app
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt install -y libgl1-mesa-glx && rm -rf /var/lib/apt/lists/*
COPY --from=unzip /unzip/antelopev2 /root/.insightface/models/antelopev2
COPY --from=builder requirements.txt ./
RUN python -m pip install --no-cache-dir -r requirements.txt
ENV FLASK_APP=app
ENV FLASK_ENV=production
ENV FLASK_DEBUG=0
COPY lr-if-alv3.sav ./
COPY app.py ./
CMD ["uvicorn", "app:app", "--host", "0.0.0.0", "--port", "8804"]
