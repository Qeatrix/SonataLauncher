import { createRouterContext, Router, Redirect } from 'hywer/x/router';

import Header from './widgets/Header/Header';
import NewsList from './widgets/NewsList/NewsList';

import './App.css';

createRouterContext(
  {
    '/': () => <Redirect path='' />,
  }
)

function App() {
  return <>
    <Header />
    <NewsList />
  </>
}

export default App
