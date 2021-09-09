import React from 'react';

class MovieDetail extends React.Component {
  componentDidMount() {
    const { location, history } = this.props;
    if (location.state === undefined) {
      history.push('/');
    }
  }
  render() {
    const { location } = this.props;
    if (location === undefined) {
      return null;
    }

    return <span>{location.state.title}</span>;
  }
}
export default MovieDetail;
