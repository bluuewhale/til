from django.db import models
from django.contrib.auth.models import User


class Order(models.Model):

    user = models.ForeignKey(User, on_delete=models.CASCADE)
    time = models.DateTimeField()
    accNo = models.CharField(max_length=10)
    code = models.CharField(max_length=10)
    price = models.IntegerField()
    qty = models.IntegerField()
    hogaType = models.CharField(max_length=2)
    orderNo = models.CharField(max_length=10)
    orderType = models.CharField(max_length=2)
    originOrderNo = models.CharField(max_length=10)
    rqName = models.CharField(max_length=30)
    scrNo = models.CharField(max_length=4)
    msg = models.TextField(blank=True)

    class Meta:
        ordering = ["-time"]

    def __str__(self):
        return self.user.username

    def is_success(self):
        return "정상처리" in self.msg

    is_success.boolean = True
