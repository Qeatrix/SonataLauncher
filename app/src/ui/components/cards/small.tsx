import { JSX } from "hywer/jsx-runtime"

interface ICardGrid {
    children: JSX.Element | JSX.Element[],
}
export function CardGrid(props: ICardGrid) {
    return (
        <div class="asd">
            {props.children}
        </div>
    )
}