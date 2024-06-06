import { ref } from 'hywer';
import './Header.css';

function Header() {
    const tabPos = ref(0);

    const data = [
        {
            "name": "Home",
            "url": "/"
        },
        {
            "name": "Discover",
            "url": "/discover"

        },
        {
            "name": "Instances",
            "url": "/instances"
        },
        {
            "name": "Tools",
            "url": "/tools"
        }
    ]

    const changeTabPos = (pos, url) => {
        console.log(`tab: ${pos} | tabPos: ${tabPos.val}`);
        tabPos.val = pos;
    }

    return (
        <>
            <div className="header">
                <div className="header-menus" id="header-menus">
                    {data.map((menu, key) => (
                        <div
                            key={key}
                            onClick={() => changeTabPos(key, menu.url)}
                            className={`header-tab ${tabPos.val === key ? 'header-active-tab' : ''}`}
                        ></div>
                    ))}
                </div>
            </div>
        </>
    )
}

export default Header;
