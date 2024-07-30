import css from './buttons.module.less';


interface IButtonProps {
    text: string,
    primary?: boolean,
    disabled?: boolean,
    onClick?: () => void,
}

export default function Button(props: IButtonProps) {
    return (
        <button
        className={
            `${css.button}
            Inter-Display-Regular
            ${props.primary == true ? css.primary : ''}`
        }
        onClick={props.onClick}
        >
            {props.text}
        </button>
    )
}