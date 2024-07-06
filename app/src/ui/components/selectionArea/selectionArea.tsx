import { JSX, Reactive, effect, ref } from "hywer/jsx-runtime";
import css from './selectionArea.module.less';
import Button from "../buttons/buttons";
import { For, Svg } from "hywer/x/html";


interface ISelectionArea {
    children: JSX.Element | JSX.Element[],
    name: string,
    dataScheme?: any,
    customEvent?: () => void,
}

export function SelectionArea(props: ISelectionArea) {
    return (
        <>
            <div className={css.Wrapper}>
                <div className={css.Name}>
                    <p className="Inter-Display-Medium">{props.name}</p>
                </div>
                <div className={css.ItemsArea}>
                    {props.children}
                </div>
            </div>
        </>
    )
}


interface ISelectionItem {
    name: string,
    onClick?: (e: any) => void,
    selected?: Reactive<boolean>,
    id?: string,
}

export function SelectionItem(props: ISelectionItem) {
    return (
        <>
            <button 
                class={props.selected?.derive(val => val == true ? css.selected : "")}
                onClick={props.onClick}
                id={props.id}
            >
                {props.name}
                {props.selected?.derive(val => {
                    if (val) {
                        return <>
                            <Svg>
                                <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                                    <path d="M20 6L9 17L4 12" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                </svg>
                            </Svg>
                        </>
                    } else {
                        return <></>;
                    }
                })}
            </button>
        </>
    )
}