import NewsList from "@/ui/widgets/NewsList/NewsList";
import Instances from "@/ui/pages/Instances/instances";
import OldInstances from "@/ui/widgets/Instaces/instances";
import UIKitDebug from "@/ui/pages/UIKitDebug/uikitdebug";

export const routeNames = {
    root: "/",
    instances: "/instances",
    uidebug: "/uidebug",
}

export const routes = {
    [routeNames.root]: () => <NewsList />,
    [routeNames.instances]: () => <OldInstances />,
    [routeNames.uidebug]: () => <UIKitDebug />,
}