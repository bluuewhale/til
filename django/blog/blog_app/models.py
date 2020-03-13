from django.db import models

# Create your models here.
class Post(models.Model):

    # author = models.ForeignKey(settings.AUTH_USER_MODEL, on_delete="cascade")
    author = models.CharField(max_length=100)
    title = models.CharField(max_length=100)
    content = models.TextField(blank=True)
    created_date = models.DateTimeField(auto_now_add=True)
    published_date = models.DateTimeField(blank=True, null=True)

    def __str__(self):
        return self.title
