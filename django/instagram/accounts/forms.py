from django import forms
from django.contrib.auth import get_user_model
from django.contrib.auth.models import User
from django.core.exceptions import ObjectDoesNotExist


class UserLoginForm(forms.Form):
    username = forms.CharField(widget=forms.TextInput(attrs={"class": "form-control"}))
    password = forms.CharField(
        widget=forms.PasswordInput(
            attrs={"class": "form-control", "placeholder": "Password"}
        )
    )

    def clean(self):  # 상속
        cleaned_data = super().clean()
        username = cleaned_data.get("username")
        try:
            user = User.objects.get(username=username)
        except ObjectDoesNotExist:
            self.add_error("username", "User doesn't exists")
            return

        password = cleaned_data.get("password")
        if not user.check_password(password):
            self.add_error("password", "Password doesn't match")

    def get_username(self):
        return self.cleaned_data.get("username")

    def get_password(self):
        return self.cleaned_data.get("password")


class SignUpForm(forms.Form):
    username = forms.CharField(widget=forms.TextInput(attrs={"class": "form-control"}))

    password = forms.CharField(
        widget=forms.PasswordInput(
            attrs={"class": "form-control", "placeholder": "Password"}
        )
    )

    password_repeat = forms.CharField(
        widget=forms.PasswordInput(
            attrs={"class": "form-control", "placeholder": "Repeat Password"}
        )
    )
    email = forms.EmailField(widget=forms.EmailInput(attrs={"class": "form-control"}))

    # username 필드에 username이 존재하는지 확인
    def clean_username(self):
        username = self.cleaned_data.get("username")
        if User.objects.filter(username=username).exists():
            raise forms.ValidationError("Duplicated Username")
        return username

    def clean_password(self):
        password = self.cleaned_data.get("password")
        password_repeat = self.cleaned_data.get("password_repeat")
        if password != password_repeat:
            raise forms.ValidationError("Password doesn't match")
        return password

    def signup(self):
        if self.is_valid():
            return User.objects.create_user(**self.cleaned_data)
        print(self.errors.as_data())
        raise forms.ValidationError("Invalid Data")
