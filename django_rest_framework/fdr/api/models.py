from django.db import models


class OHLC(models.Model):

    ticker = models.CharField(max_length=10)
    date = models.CharField(max_length=10)
    open = models.IntegerField()
    high = models.IntegerField()
    low = models.IntegerField()
    close = models.IntegerField()
    volume = models.IntegerField()
    change = models.FloatField()

    class Meta:
        unique_together = ["ticker", "date"]
