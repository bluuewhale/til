import Joi from 'joi';
import { ObjectId } from 'mongodb';

export interface Movie {
  _id: ObjectId;
  awards: {
    wins: number;
    nominations: number;
    text: string;
  };
  cast: string[];
  countries: string[];
  directors: string[];
  fullplot: string;
  genres: string[];
  imdb: {
    rating: number;
    votes: number;
    id: number;
  };
  languages: string[];
  lastUpdated: Date;
  released: Date;
  runtime: number;
  title: string;
}

export type MovieWithoutId = Omit<Movie, '_id'>;

export const movieSchema = Joi.object().keys({
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
