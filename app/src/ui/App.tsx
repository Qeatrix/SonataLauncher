import { ref } from "hywer"

import Header from "@/ui/widgets/Header/Header"
import NewsList from "@/ui/widgets/NewsList/NewsList"

import css from "./App.module.less"
import "./App.css"
import { Router, createRouterContext } from "hywer/x/router"
import { routes } from "./routes"

const App = () => {
  createRouterContext(routes)

  return <>
    <Header />
    <Router />
  </>
}

export default App;
