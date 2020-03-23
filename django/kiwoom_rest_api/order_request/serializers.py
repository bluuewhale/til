from rest_framework import serializers
from .models import Order


class OrderSerializer(serializers.Serializer):
    acc_no = serializers.CharField(max_length=10)
    code = serializers.CharField(max_length=10)
    price = serializers.IntegerField()
    qty = serializers.IntegerField()
    time = serializers.DateTimeField()
    hogaType = serializers.CharField(max_length=2)
    order_no = serializers.CharField(max_length=10)
    order_type = serializers.CharField(max_length=2)
    origin_order_no = serializers.CharField(max_length=10)
    rq_name = serializers.CharField(max_length=30)
    screen_no = serializers.CharField(max_length=4)
    msg = serializers.CharField()

    def create(self, validated_data):
        return Order(**validated_data)
