const router = require('express').Router();
const mongo = require('mongodb');
const Joi = require('joi');
const getClient = require('../../db').get;

const DB = 'sample_mflix';
const COLLECTION = 'movies';

async function getCollection() {
  return await getClient().db(DB).collection(COLLECTION);
}

// GET /api/movies
router.get('/', async function (req, res) {
  const movieCol = await getCollection();

  const query = { runtime: { $lt: 15 } };
  const options = {
    // sort returned documents in ascending order by title (A->Z)
    sort: { title: 1 },
    // Include only the `title` and `imdb` fields in each returned document
    projection: {
      _id: 0,
      awards: 1,
      cast: 1,
      countries: 1,
      directors: 1,
      fullplot: 1,
      genres: 1,
      imdb: 1,
      languages: 1,
      lastUpdated: 1,
      released: 1,
      runtime: 1,
      title: 1,
    },
  };

  const movies = await movieCol.find(query, options).toArray();

  res.json(movies);
});

// POST /api/movies
const postSchema = Joi.object().keys({
  genres: Joi.array().items(Joi.string()).required(),
  runtime: Joi.number().required(),
  cast: Joi.array().items(Joi.string()).required(),
  title: Joi.string().required(),
  fullplot: Joi.string().required(),
  countries: Joi.array().items(Joi.string()).required(),
  languages: Joi.array().items(Joi.string()),
  released: Joi.string().required(),
  directors: Joi.array().items(Joi.string()).required(),
  awards: Joi.object({
    wins: Joi.number().required(),
    nominations: Joi.number().required(),
    text: Joi.string().required(),
  }).required(),
  imdb: Joi.object({
    rating: Joi.number().required(),
    votes: Joi.number().required(),
    id: Joi.number().required(),
  }).required(),
});

router.post('/', async function (req, res) {
  const { error, value } = postSchema.validate(req.body);
  if (error) {
    throw error;
  }

  const movieCol = await getCollection();
  const result = await movieCol.insertOne(value);
  res.json({
    id: result.insertedId,
    text: 'ok',
  });
});

router.get('/:id', async function (req, res) {
  const query = { _id: new mongo.ObjectId(req.params.id) };
  const options = {
    projection: {
      _id: 0,
      awards: 1,
      cast: 1,
      countries: 1,
      directors: 1,
      fullplot: 1,
      genres: 1,
      imdb: 1,
      languages: 1,
      lastUpdated: 1,
      released: 1,
      runtime: 1,
      title: 1,
    },
  };

  const movieCol = await getCollection();
  res.json(await movieCol.findOne(query, options));
});

module.exports = router;
