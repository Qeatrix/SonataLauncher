import { JSX, Reactive, derive, effect, ref } from 'hywer/jsx-runtime';
import css from './window.module.less';
import { For } from 'hywer/x/html';
import { gsap, ScrollToPlugin } from 'gsap/all';


interface IWindow {
    children: JSX.Element | JSX.Element[],
    name: string,
    minimize?: boolean,
    maximize?: boolean,
}

export function Window(props: IWindow) {
    return (
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
            {props.children}
        </div>
    )
}


interface IWindowControls {
    children: JSX.Element | JSX.Element[],
}

export function WindowControls(props: IWindowControls) {
    return (
        <div className={css.WindowControls}>
            {props.children}
        </div>
    )
}


interface IFlexBox {
    children: JSX.Element | JSX.Element[],
}

export function FlexBox(props: IFlexBox) {
    return (
        <div className={css.FlexBox}>
            {props.children}
        </div>
    )
}


interface IContentStack {
    children: JSX.Element[],
    showIndex: Reactive<number>,
}

export function ContentStack(props: IContentStack) {
    const contentHeight = ref<number>(0);
    const OldIndex = ref<number | null>(null);
    const currentIndex = ref<number>(props.showIndex.val);
    const Loaded = ref(false);
    gsap.registerPlugin(ScrollToPlugin);

    const asd = (i: number) => {
        console.log(`key: ${i} | value: ${props.showIndex.val} | ${props.showIndex.derive(val => val === i)}`);
    }

    effect(() => {
        WindowContentWidth();
    }, [])

    const WindowContentWidth = () => {
        const animContent = document.getElementById('WindowContent');
        let scrollValue: any = "max";

        // console.log(animContent?.children);

        if (props.showIndex.oldVal < props.showIndex.val) {
            // console.log("Going forward");
        } else if (props.showIndex.oldVal > props.showIndex.val) {
            // console.log("Going previous");
            scrollValue = 0;

            if (animContent) {
                animContent.scrollLeft = animContent?.scrollWidth;
                console.log(animContent.children[props.showIndex.oldVal]);
            }
        }


        if (animContent) {
            gsap.to(animContent, {
                scrollTo: { x: scrollValue },
                height: (animContent.children[props.showIndex.val]?.children[0] as HTMLDivElement).offsetHeight + 30,
                ease: 'power1.inOut',
                duration: 0.35,
                onComplete: () => {
                    // animContent.children[props.showIndex.oldVal].children[0]?.parentElement?.remove();

                    let oldComponent = document.getElementById(`Content[${props.showIndex.oldVal}]`);
                    oldComponent?.remove();
                }
            })
        }
    }

    const handleChange = () => {
        if (Loaded.val === false) {
            Loaded.val = true;
        } else {
            OldIndex.val = currentIndex.val;
            setTimeout(() => {
                WindowContentWidth();
            }, 0)
        }
    }

    return (
        <div id="WindowContent" class={css.WindowContent}>
            {
                derive(([val]) => {
                    handleChange();

                    return <For in={props.children}>
                            {(item, i) => {
                                return <>
                                    {
                                        i === val.val || i === val.oldVal ? (
                                            <div class={css.Content} id={`Content[${i}]`}>
                                                {item}
                                            </div>
                                        ) : null
                                    }
                                </>
                            }}
                        </For>
                }, [props.showIndex])
            }
        </div>
    )
}