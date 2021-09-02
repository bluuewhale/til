import { Router, Request, Response, NextFunction } from 'express';
import { ObjectId } from 'mongodb';
import { asyncWrapper } from '../../lib/utils';
import { verifyToken } from '../jwt';
import { getClient } from '../../db';
import { Movie, movieSchema, MovieWithoutId } from '../../types/movie';
import { MyError } from '../../errors';

const DB = 'sample_mflix';
const COLLECTION = 'movies';

async function getMovieCollection() {
  return (await getClient()).db(DB).collection<Movie>(COLLECTION);
}

// GET /api/movies
const router = Router();

router.get('/', verifyToken, async (req, res) => {
  const movieCol = await getMovieCollection();

  const movies = await movieCol
    .find({ runtime: { $lt: 15 } })
    .sort({ title: 1 })
    .project<MovieWithoutId>({ _id: 0 })
    .toArray();

  res.json(movies);
});

// POST /api/movies
router.post(
  '/',
  verifyToken,
  asyncWrapper(async (req: Request, res: Response): Promise<void> => {
    const movieCol = await getMovieCollection();
    const value = await movieSchema.validateAsync(req.body);
    const result = await movieCol.insertOne(value);
    res.json({
      id: result.insertedId,
      text: 'ok',
    });
  })
);

router.get(
  '/:id',
  verifyToken,
  asyncWrapper(async (req: Request, res: Response, next: NextFunction): Promise<void> => {
    const movieCol = await getMovieCollection();
    const movie = await movieCol.findOne(
      { _id: new ObjectId(req.params.id) },
      { projection: { _id: 0 } }
    );

    if (!movie) {
      return next(new MyError('movie not exists', 404));
    }

    res.json(movie);
  })
);

export default router;
