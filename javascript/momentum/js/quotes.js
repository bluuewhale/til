const quotes = [
  { quote: 'He not busy being born is busy dying.', author: 'Bob Dylan' },
  {
    quote:
      'The supreme irony of life is that hardly anyone gets out of it alive.',
    author: 'Robert Heinlein',
  },
  {
    quote:
      'Dost thou love life? Then do not squander time, for that is the stuff life is made of.',
    author: 'Benjamin Franklin',
  },
  { quote: 'Life is a long lesson in humility.', author: 'ames M. Barrie' },
  {
    quote:
      'The tragedy of life is what dies in the hearts and souls of people while they live.',
    author: 'Albert Einstein',
  },
  {
    quote:
      'Life is like playing a violin in public and learning the instrument as one goes on.',
    author: 'Samuel Butler',
  },
  {
    quote:
      'By nature, men are nearly alike; by practice, they get to be wide apart.',
    author: 'Confucius',
  },
  {
    quote:
      'Security is mostly a superstition. It does not exist in nature.... Life is either a daring adventure or nothing.',
    author: 'Helen Keller',
  },
  {
    quote: 'Life is a moderately good play with a badly written third act.',
    author: 'Truman Capote',
  },
  {
    quote:
      'Thanks to the Interstate Highway System, it is now possible to travel from coast to coast without seeing anything.',
    author: 'Charles Kuralt',
  },
];

function getRandomQuote(quotes) {
  const randomIdx = Math.floor(Math.random() * quotes.length);
  return quotes[randomIdx];
}

const quoteSpan = document.querySelector('#quote-text');
const authorSpan = document.querySelector('#quote-author');

const todaysQuote = getRandomQuote(quotes);
quoteSpan.innerText = todaysQuote.quote;
authorSpan.innerText = todaysQuote.author;
