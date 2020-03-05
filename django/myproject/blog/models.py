from django.conf import settings
from django.db import models


class Post(models.Model):

    author = models.CharField(max_length=100)
    title = models.CharField(max_length=100)  # 글자수가 제한되 text field
    content = models.TextField(blank=True)  # 많은 양의 문자열 저장
    created_date = models.DateTimeField(auto_now_add=True)
    published_date = models.DateTimeField(blank=True, null=True)

    def __str__(self):

        return self.title
