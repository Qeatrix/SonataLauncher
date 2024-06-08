import NewsList from "./NewsList/NewsList"
import Instances from "./Instaces/instances"

export const routeNames = {
    root: "/",
    instances: "/instances",
}

export const routes = {
    [routeNames.root]: () => <NewsList />,
    [routeNames.instances]: () => <Instances />,
}