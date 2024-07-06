import css from './input.module.less';

interface IInput {
    name: string,
    placeholder?: string,
    defaultValue?: string,
}

export default function Input(props: IInput) {
    return (
        <>
            <div className={css.Wrapper}>
                <p className="Inter-Display-Medium">{props.name}</p>
                <input></input>
            </div>
        </>
    )
}