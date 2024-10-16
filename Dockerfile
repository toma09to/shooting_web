FROM python:3.13

WORKDIR /var/www/app

COPY . /var/www/app

RUN python3 -m pip install -r requirements.txt

CMD ["python3", "server.py"]
