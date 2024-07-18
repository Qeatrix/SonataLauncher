import TranslationStore from "@/data/localization/store";
import { ProgressMessage, ProgressMessageFinish, ProgressStatuses, ProgressTargetsList } from "@/data/types";
import { ProgressBarOptions } from "electron/renderer";
import { JSX, Reactive, derive, ref } from "hywer/jsx-runtime";
import css from './style.module.less';
import localization from '@/data/localization/en.json';
import { For, Svg } from "hywer/x/html";
import Store from "@/data/store";
import { gsap } from 'gsap/all';


interface IProgressDisplay {
    children?: JSX.Element | JSX.Element[],
    progressItems: Reactive<ProgressTargetsList>,
    message: Reactive<ProgressMessage>
}

export function ProgressDisplay(props: IProgressDisplay) {
    const activeObject = ref<HTMLElement | null>(null);
    const componentId = Store.makeId(6);
    const currentWorkId = ref<number | null>(null);
    const completedElements = ref<number[]>([]);

    props.message.sub = (val) => {
        // Get the id of the stage with last processed element
        currentWorkId.val = props.progressItems.val.ids_list.indexOf(val.data.stage);

        if (val.data.status === ProgressStatuses.COMPLETED) {
            const lastProcessedElement = document.getElementById(`ProgressItem-${componentId}-${currentWorkId.val}`);

            if (lastProcessedElement) {
                completedElements.val.push(currentWorkId.val);

                gsap.to(lastProcessedElement, {
                    color: 'green',
                    ease: 'power1.inOut',
                    duration: 0.35,
                })
            }
        }
    }

    return (
        <>
            <div class={css.ProgressDisplay}>
                {derive(([progressItems]) => {
                    return <For in={progressItems.val.ids_list}>
                        {(stage_name, i) => {
                            return <>
                                <div class={css.Item} id={`ProgressItem-${componentId}-${i}`}>
                                    {
                                        props.message.derive(val => {
                                            // TODO: Fix svg icon handling on finishing last download
                                            if (currentWorkId.val !== null && currentWorkId.val > i) {
                                                return <>
                                                    <Svg>
                                                        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                                                            <path d="M20 6L9 17L4 12" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                                        </svg>
                                                    </Svg>
                                                </>
                                            } else {
                                                return <></>
                                            }
                                        })
                                    }
                                    {
                                        props.message.derive(val => {
                                            // TODO: Fix naming on finishing last download
                                            if (val.data.status === ProgressStatuses.INPROGRESS && currentWorkId.val === i) {
                                                return <>
                                                    <p class={css.Name}>{TranslationStore.t(`inprogress.${stage_name}`)}</p>
                                                </>
                                            } else {
                                                return <>
                                                    <p class={css.Name}>{TranslationStore.t(stage_name)}</p>
                                                </>
                                            }
                                        })
                                    }
                                    <div class={css.Status}>
                                        {
                                            props.message.derive(val => {
                                                if (currentWorkId.val !== null && currentWorkId.val > i) {
                                                    return <p>Completed</p>;
                                                } else if (currentWorkId.val === i) {
                                                        return <p>In Progress</p>;
                                                } else if (currentWorkId.val !== null && currentWorkId.val < i) {
                                                    return <p>Pending</p>;
                                                } else {
                                                    return <p>Unknown</p>;
                                                }
                                            })
                                        }
                                    </div>
                                </div>
                            </>
                        }}
                    </For>
                }, [props.progressItems])}
            </div>
        </>
    )
}


// TODO: It probably won't be needed anymore
interface IProgressItem {
    name_id: string,
    message?: Reactive<ProgressMessage | ProgressMessageFinish>,
    percentage?: number,
    id?: string,
}

export function ProgressItem(props: IProgressItem) {
    setTimeout(() => {
        props.message?.derive(val => {
            // console.log("Message Updated");
        });
    }, 0);

    return (
        <>
            <div class={css.Item} id={props.id}>
                {
                    props.message?.derive(val => {
                        if (val.data.status === ProgressStatuses.INPROGRESS) {
                            return <><p class={css.Name}>{TranslationStore.t(`inprogress.${props.name_id}`)}</p></>
                        } else {
                            return <><p class={css.Name}>{TranslationStore.t(props.name_id)}</p></>
                        }
                    })
                }
                <div class={css.Status}>
                    {
                        props.message?.derive(val => {
                            {switch(val.data.status) {
                                case ProgressStatuses.PENDING:
                                    return <p>Pending</p>;
                                case ProgressStatuses.INPROGRESS:
                                    return <p>In Progress</p>;
                                case ProgressStatuses.COMPLETED:
                                    return <p>Completed</p>;
                                case ProgressStatuses.FAILED:
                                    return <p>FAILED</p>;
                                default:
                                    return <p>Unknown</p>;
                            }}
                        })
                    }
                </div>
            </div>
        </>
    )
}