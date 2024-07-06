import { ref } from 'hywer';
import { routeNames } from '@/ui/routes';

import './Header.css';
import { navigateTo } from 'hywer/x/router';

function Header() {
    const tabPos = ref(0);

    const changeTabPos = (pos: number, url: string) => {
        navigateTo(url);
        tabPos.val = pos;
    }

    return (
        <>
            <div className="header">
                <div className="header-menus" id="header-menus">
                    {Object.entries(routeNames).map(([key, value], index) => (
                        <div
                            key={key}
                            onClick={() => changeTabPos(index, key)}
                            className={tabPos.derive(val => `header-tab ${val === index ? 'header-active-tab' : ''}`)}
                        ></div>
                    ))}
                </div>
            </div>
        </>
    )
}

export default Header;
