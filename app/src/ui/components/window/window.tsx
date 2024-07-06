import { JSX } from 'hywer/jsx-runtime';
import css from './window.module.less';

interface IWindow {
    children: JSX.Element | JSX.Element[],
    name: string,
    minimize?: boolean,
    maximize?: boolean,
}

export function Window(props: IWindow) {
    return (
        <>
            <div className={css.Window}>
                <div className={css.WindowHeader}>
                    <div className={css.Name}>
                        <p className="Inter-Display-Semibold">{props.name}</p>
                    </div>
                    <div className={css.ControlsWrapper}>
                        <button className={css.Minimize}></button>
                        <button className={css.Maximize}></button>
                    </div>
                </div>
                <div className={css.Content}>
                    {props.children}
                </div>
            </div>
        </>
    )
}


interface IWindowControls {
    children: JSX.Element | JSX.Element[],
}

export function WindowControls(props: IWindowControls) {
    return (
        <>
            <div className={css.WindowControls}>
                {props.children}
            </div>
        </>
    )
}


interface IFlexBox {
    children: JSX.Element | JSX.Element[],
}
export function FlexBox(props: IFlexBox) {
    return (
        <>
            <div className={css.FlexBox}>
                {props.children}
            </div>
        </>
    )
}