import React from 'react';
import axios from 'axios';
import Movie from '../components/Movie';
import './Home.css';

class Home extends React.Component {
  state = {
    isLoading: true,
    movies: [],
  };

  updateState = (key, val) => {
    this.setState({
      ...this.state,
      [key]: val,
    });
  };

  componentDidMount() {
    this.getMovies().then(async () => this.updateState('isLoading', false));
  }

  render() {
    const { isLoading, movies } = this.state;
    return (
      <section className="container">
        {isLoading ? (
          <div className="loader">
            <span className="loader__text">Loading...</span>
          </div>
        ) : (
          <div className="movies">
            {movies.map((movie) => {
              const { id, title, year, summary, medium_cover_image, genres } =
                movie;
              return (
                <Movie
                  key={id}
                  id={id}
                  title={title}
                  year={year}
                  summary={summary}
                  poster={medium_cover_image}
                  genres={genres}
                />
              );
            })}
          </div>
        )}
      </section>
    );
  }

  getMovies = async (cb) => {
    const response = await axios.get(
      'https://yts-proxy.now.sh/list_movies.json?sort_by=rating'
    );
    const movies = response.data.data.movies;

    this.setState({ movies });
  };
}

export default Home;
