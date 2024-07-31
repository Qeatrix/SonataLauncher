import { JSX, Reactive } from "hywer/jsx-runtime";
import css from "./tabs.module.less";
import { For } from "hywer/x/html";


interface ITabStack {
    children: JSX.Element | JSX.Element[],
    tabnames: string[],
    selection: Reactive<number>
}

export function TabStack(props: ITabStack) {
    const changeSelectedTab = (num: number) => {
        props.selection.val = num;
    }

    return (
        <>
            <div class={css.Tabstack}>
                <For in={props.tabnames}>
                    {(item, i) => {
                        return <>
                            <Tab name={props.tabnames[i]} onClick={() => changeSelectedTab(i)} selection={props.selection} tabNumber={i} />
                        </>
                    }}
                </For>
            </div>
            {props.selection.derive(val => {
                return <For in={props.children}>
                    {(item, i) => {
                        if (val === i) {
                            return <>{item}</>
                        }
                    }}
                </For>
            })}
        </>
    )
}


interface ITab {
    name: string,
    onClick: () => any,
    selection: Reactive<number>,
    tabNumber: number,
}

export function Tab(props: ITab) {
    props.selection.sub = (val) => {
        console.log(`Selected: ${val} | TabNumber: ${props.tabNumber}`);
    }

    return (
        <>
            {props.selection.derive(val => {
                console.log(val === props.tabNumber ? `${css.Tab}` : `${css.Tab} ${css.selected}`);
                return <button
                    onClick={props.onClick} class={val === props.tabNumber ? `${css.Tab} ${css.selected}` : `${css.Tab}`}>{props.name}</button>
            })}
        </>
    )
}
