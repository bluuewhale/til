from __future__ import absolute_import
from celery import Celery
from gevent import monkey

app = Celery(
    "test_celery",
    broker="amqp://guest:guest@localhost//",
    backend="rpc://",
    include=["test_celery.tasks"],
)

app.conf.update(
    task_serializer="json",
    # accept_content=['json'],  # Ignore other content
    result_serializer="json",
    timezone="Asia/Seoul",
    enable_utc=False,
)
