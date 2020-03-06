from rest_framework import serializers
from movies.models import Movie


class MovieListSerializer(serializers.ModelSerializer):
    class Meta:
        model = Movie  # 모델 설정
        fields = ("id", "title", "genre", "year")  # 필드 설정


class MovieDetailSerializer(serializers.ModelSerializer):
    class Meta:
        model = Movie  # 모델 설정
        fields = ("id", "title", "genre", "year")  # 필드 설정
