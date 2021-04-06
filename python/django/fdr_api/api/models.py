from django.db import models


class OHLC(models.Model):

    ticker = models.CharField(max_length=10)
    date = models.DateField()
    open = models.FloatField()
    high = models.FloatField()
    low = models.FloatField()
    close = models.FloatField()
    volume = models.FloatField()
    change = models.FloatField(null=True)  # 맨 첫날에는 None

    class Meta:
        unique_together = ["ticker", "date"]
