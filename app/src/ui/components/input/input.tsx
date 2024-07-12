import css from './input.module.less';

interface IInput {
    name: string,
    placeholder?: string,
    defaultValue?: string,
    search?: boolean,
    onInput?: (e: InputEvent) => void,
    id: string
}

export default function Input(props: IInput) {
    return (
        <>
            <div className={css.Wrapper}>
                <p className="Inter-Display-Medium">{props.name}</p>
                <input placeholder={props.placeholder} value={props.defaultValue} id={props.id} onInput={props.onInput}></input>
            </div>
        </>
    )
}