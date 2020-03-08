from rest_framework import serializers

from .models import OHLC


class OHLCReadSerializer(serializers.Serializer):

    # id = serializers.IntegerField()
    # ticker = serializers.CharField(max_length=10)  # YYYY-MM-DD
    date = serializers.DateField()

    open = serializers.FloatField()
    high = serializers.FloatField()
    low = serializers.FloatField()
    close = serializers.FloatField()
    volume = serializers.FloatField()
    change = serializers.FloatField()

    read_only_fields = ["date", "open", "high", "low", "close", "volume", "change"]


class OHLCCreateSerializer(serializers.Serializer):

    ticker = serializers.CharField(max_length=10)  # YYYY-MM-DD
    date = serializers.DateField()

    open = serializers.FloatField()
    high = serializers.FloatField()
    low = serializers.FloatField()
    close = serializers.FloatField()
    volume = serializers.FloatField()
    change = serializers.FloatField()

    def create(self, validated_data):
        " validation을 거친 데이터로 새 OHLC 인스턴스를 생성하여 반환"
        return OHLC.objects.create(**validated_data)
