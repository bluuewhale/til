from rest_framework import serializers
from django.http import HttpResponse, JsonResponse

from .models import OHLC


class OHLCSerializerFactory:
    """ 원하는 Field만 선택하는 Serializer 클래스 생성 팩토리  """

    @staticmethod
    def create(using_fields=None, is_read_only=False):

        if using_fields is not None:  # default: all
            total_fields = BaseSerializer.get_declared_fields()
            del_fields = list(set(total_fields) - set(using_fields))

            for f in del_fields:  #
                del BaseSerializer._declared_fields[f]

        return BaseSerializer


class BaseSerializer(serializers.Serializer):

    ticker = serializers.CharField(max_length=10)
    date = serializers.DateField()  # YYYY-MM-DD
    open = serializers.FloatField()
    high = serializers.FloatField()
    low = serializers.FloatField()
    close = serializers.FloatField()
    volume = serializers.FloatField()
    change = serializers.FloatField()

    read_only_fields = []

    def create(self, validated_data):
        " validation을 거친 데이터로 새 OHLC 인스턴스를 생성하여 반환"
        return OHLC.objects.create(**validated_data)

    @classmethod
    def get_declared_fields(cls):
        return list(getattr(BaseSerializer, "_declared_fields").keys())

    def save_data(self):
        """ create(save) data instance in DB
        data must be validable else return HTTP Response 400

        Parameters
        ----------
        data : all
            validatble data

        Returns
        -------
        django.http.HttpResponse
            201(success) or 400(fail)
        """

        if self.is_valid():
            try:
                self.save()
            except IntegrityError:
                pass
        else:
            # return HttpResponse(status=400)
            return JsonResponse(self.errors, safe=False)  # for debug only

        return HttpResponse(status=201)
