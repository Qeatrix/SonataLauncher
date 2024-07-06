import { gsap } from 'gsap';
import css from './buttons.module.less';


interface IButtonProps {
    text: string,
    primary?: boolean,
    disabled?: boolean
}

export default function Button(props: IButtonProps) {
    return (
        <>
            <button
            className={
                `${css.button}
                Inter-Display-Regular
                ${props.primary == true ? css.primary : ''}`
            }
            >
                {props.text}
            </button>
        </>
    )
}