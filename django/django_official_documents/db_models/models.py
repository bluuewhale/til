from django.db import models

""" Fields """


class Person(models.Model):
    # char field must have max_length argument
    name = models.CharField(max_length=50)

    def __str__(self):
        return self.name


class Customer(Person):
    # choices : ('saved in db', 'how it shows on admin page')
    shirt_size = models.CharField(
        max_length=1, choices=[("S", "Small"), ("M", "Medium"), ("L", "Large")],
    )

    NATIONAL_CHOICES = (("US", "The United States"), ("KR", "South Korea"))
    nationality = models.CharField(max_length=2, choices=NATIONAL_CHOICES)


class Runner(Person):
    MEDAL_TYPE = models.TextChoices("MEDAL_TYPE", "GOLD SILVER BRONZE")
    medal = models.CharField(blank=True, choices=MEDAL_TYPE.choices, max_length=10)


class CollegeStudent(Person):
    year_in_school = models.CharField(
        max_length=2,
        choices=[
            ("FR", "Freshman"),
            ("SO", "Sophomore"),
            ("JR", "Junior"),
            ("SR", "Senior"),
            ("GR", "Graduate"),
        ],
    )


""" relationship """

# many-to-one relationship
class Musician(models.Model):
    instrument = models.CharField(max_length=100)


class Album(models.Model):
    artist = models.ForeignKey(Musician, on_delete=models.CASCADE)
    name = models.CharField(max_length=100)
    release_date = models.DateField()
    num_start = models.IntegerField()


# many-to-many relationship
class Topping(models.Model):
    name = models.CharField(max_length=50)


class Pizza(models.Model):
    toppings = models.ManyToManyField(Topping)
    """ Generally, ManyToManyField instances should go 
    in the object that’s going to be edited on a form. 
    In the above example, toppings is in Pizza 
    (rather than Topping having a pizzas ManyToManyField) 
    because it’s more natural to think about a pizza having
    toppings than a topping being on multiple pizzas. 
    The way it’s set up above, the Pizza form would let 
    users select the toppings.
    """


class Publication(models.Model):
    title = models.CharField(max_length=30)

    class Meta:
        ordering = ["title"]

    def __str__(self):
        return self.title


class Article(models.Model):
    headline = models.CharField(max_length=100)
    publications = models.ManyToManyField(Publication)

    class Meta:
        ordering = ["headline"]

    def __str__(self):
        return self.headline


class Group(models.Model):  # Extra fields on many-to-many relationships

    name = models.CharField(max_length=128)
    members = models.ManyToManyField(Person, through="Membership")

    def __str__(self):
        return self.name


class Membership(models.Model):
    person = models.ForeignKey(Person, on_delete=models.CASCADE)
    group = models.ForeignKey(Group, on_delete=models.CASCADE)
    date_joined = models.DateField()
    invite_reason = models.CharField(max_length=64)

    def __str__(self):
        return f"{str(self.person)}__{str(self.group)}"


# One-to-one relationship

"""
OneToOneField requires a positional argument: the class to which the model is related.

For example, if you were building a database of "places", you would build pretty standard
 stuff such as address, phone number, etc. in the database. Then, if you wanted to build 
 a database of restaurants on top of the places, instead of repeating yourself and 
 replicating those fields in the Restaurant model, you could make Restaurant have a 
 OneToOneField to Place (because a restaurant "is a" place; in fact, to handle this you'd 
 typically use inheritance, which involves an implicit one-to-one relation).
"""


class Place(models.Model):
    zipcode = models.CharField(max_length=30)


class Restaurant(models.Model):
    place = models.OneToOneField(Place, on_delete=models.CASCADE)


# Meta Options
"""
Model metadata is "anything that's not a field", such as ordering options (ordering), 
database table name (db_table), or human-readable singular and plural names 
(verbose_name and verbose_name_plural). 

None are required, and adding class Meta to a model is completely optional.
"""


class Ox(models.Model):
    horn_length = models.IntegerField()

    class Meta:
        ordering = ["horn_length"]
        verbose_name_plural = "oxen"


# Model attributes
""" object (default Manager)

The most important attribute of a model is the Manager. It's the interface through which
database query operations are provided to Django models and is used to retrieve the instances 
from the database. If no custom Manager is defined, the default name is objects. 

Managers are only accessible via model classes, not the model instances.
"""


# Model methods
"""

"""

