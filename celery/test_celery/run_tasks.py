import time

from test_celery.tasks import working


def start_work(n: int):
    for i in range(n):
        result = working.delay(i)


if __name__ == "__main__":
    start_work(n=24)
