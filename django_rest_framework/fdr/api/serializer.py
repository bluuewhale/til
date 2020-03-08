from rest_framework import serializers


class OHLCSerializer(serializers.Serializer):

    ticker = serializers.CharField()
    date = serializers.CharField()  # YYYY-MM-DD
    open = serializers.IntegerField()
    high = serializers.IntegerField()
    low = serializers.IntegerField()
    close = serializers.IntegerField()
    volume = serializers.IntegerField()
    change = serializers.FloatField()
