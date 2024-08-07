import { JSX, Reactive, effect, ref } from "hywer/jsx-runtime";
import css from './selectionArea.module.less';
import Button from "../buttons/buttons";
import { For, Svg } from "hywer/x/html";
import Input from "../input/input";
import Search from "./search";


interface ISelectionArea {
    children: JSX.Element | JSX.Element[] | Reactive<JSX.Element> | Reactive<JSX.Element[]>,
    name: string,
    dataScheme?: any,
    customEvent?: () => void,
    searchBar?: boolean,

    onValueChange: (value: string, url: string) => any
    selectedValue: Reactive<string>
}

export function SelectionArea(props: ISelectionArea) {
    const searchQuery = ref("")

    const onInputQuery = (e: InputEvent) =>  {
        const input = e.target as HTMLInputElement
        if(/^\s/.test(input.value)) input.value = '';

        searchQuery.val = input.value

        console.log(input.value.length)
    }

    return (
        <div className={css.Wrapper}>
            <div className={css.Name}>
                <p className="Inter-Display-Medium">{props.name}</p>
                {
                    props.searchBar && <Input id="searchVersions" name="" onInput={onInputQuery} />
                }
            </div>
            {
                searchQuery.derive((val) => {
                    if (val.length == 0) {
                        return (
                            <div className={css.ItemsArea}>
                                {props.children}
                            </div>
                        )
                    } else {
                        return (
                            <div className={css.ItemsArea}>
                                <Search selectedValue={props.selectedValue} query={val} onSelect={props.onValueChange}/>
                            </div>
                        )
                    }
                })
            }
        </div>
    )
}


interface ISelectionItem {
    name: string,
    onClick?: (e: Event) => void,
    selected?: boolean,
    id?: string,
}

export function SelectionItem(props: ISelectionItem) {
    const asd = (e: Event) => {

        props.onClick && props.onClick(e);

        const button = e.target as HTMLButtonElement

        button.parentElement?.querySelectorAll(`button.${css.SelectedButton}`).forEach((elem) => {
            elem.classList.remove(css.SelectedButton)
        })

        button.classList.add(css.SelectedButton)
    }

    return (
        <button 
            className={props.selected && props.selected == true ? css.SelectedButton : ""}
            onClick={asd}
            id={props.id}
        >
            {props.name}
            <Svg>
                <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M20 6L9 17L4 12" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
            </Svg>
        </button>
    )
}