from django.contrib.auth.models import User
from django.db import models
from django.urls import reverse


class Photo(models.Model):
    author = models.ForeignKey(User, on_delete=models.CASCADE, related_name="user")
    text = models.TextField(blank=True)
    image = models.ImageField(upload_to="timeline_photo/%Y/%m/%d")
    created = models.DateTimeField(auto_now_add=True)  # update field only when created
    updated = models.DateTimeField(auto_now=True)  # update field on every save

    class Meta:
        ordering = ["-created"]  # 최신순으로 정렬

    def __str__(self):
        return f"text : {self.text}"

    def get_absolute_url(self):
        return reverse("photo:detail", args=(self.id,))

