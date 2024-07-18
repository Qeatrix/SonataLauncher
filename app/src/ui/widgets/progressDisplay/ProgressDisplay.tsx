import TranslationStore from "@/data/localization/store";
import { ProgressMessage, ProgressMessageFinish, ProgressStatuses, ProgressTargetsList } from "@/data/types";
import { ProgressBarOptions } from "electron/renderer";
import { JSX, Reactive, derive, ref } from "hywer/jsx-runtime";
import css from './style.module.less';
import localization from '@/data/localization/en.json';
import { For, Svg } from "hywer/x/html";
import Store from "@/data/store";
import { gsap } from 'gsap/all';
import DoneIcon from './assets/Done';


interface IProgressDisplay {
    children?: JSX.Element | JSX.Element[],
    progressItems: Reactive<ProgressTargetsList>,
    message: Reactive<ProgressMessage>
}

export function ProgressDisplay(props: IProgressDisplay) {
    const componentId = Store.makeId(6);
    const currentWorkId = ref<number | null>(null);
    const completedElements = ref<HTMLElement[]>([]);

    props.message.sub = (val) => {
        // Get the id of the stage with last processed element
        currentWorkId.val = props.progressItems.val.ids_list.indexOf(val.data.stage);

        updateWorkName(val);

        if (val.data.status === ProgressStatuses.COMPLETED) {
            const lastProcessedElement = getLastProcessedElement();

            if (lastProcessedElement) {
                completedElements.val.push(lastProcessedElement);

                const workName = completedElements.val[currentWorkId.val].getElementsByTagName("p")[0];

                if (workName) {
                    setTimeout(() => {
                        gsap.to(workName, {
                            fontVariationSettings: "'wght' " + 900,
                            ease: 'power1.inOut',
                            duration: 0.35,
                        })
                    });
                }
            }

            currentWorkId.val++;
        }
    }

    currentWorkId.sub = (val) => {
        const lastProcessedElement = document.getElementById(`ProgressItem-${componentId}-${currentWorkId.val}`);

        if (lastProcessedElement) {
            gsap.to(lastProcessedElement, {
                opacity: 1,
                ease: 'power1.inOut',
                duration: 2,
            })
        }
    }

    const updateWorkName = (val: ProgressMessage) => {
        const lastProcessedElement = getLastProcessedElement();
        const workName = lastProcessedElement?.getElementsByTagName("p")[0];

        if (workName) {
            if (val.data.status === ProgressStatuses.INPROGRESS) {
                workName.innerText = TranslationStore.t(`inprogress.${val.data.stage}`);
            } else {
                workName.innerText = TranslationStore.t(`${val.data.stage}`);
            }
        }
    }

    const getLastProcessedElement = () => {
        return document.getElementById(`ProgressItem-${componentId}-${currentWorkId.val}`);
    }

    return (
        <>
            <div class={css.ProgressDisplay}>
                {
                    derive(([progressItems]) => {
                        if (progressItems.val.ids_list.length === 0) {
                            return <>
                                <p>asd</p>
                            </>
                        } else {
                            return <For in={progressItems.val.ids_list}>
                                {(stage_name, i) => {
                                    console.warn("PROGRESS ITEM: " + stage_name);
                                    return <>
                                        <div style="display:none">{stage_name}</div>
                                        <div className={
                                            currentWorkId.derive(val => val !== null && val < i ? `${css.Item} ${css.Pending}` : `${css.Item}`)
                                        } id={`ProgressItem-${componentId}-${i}`}>
                                            {
                                                currentWorkId.derive(val => {
                                                    if (val !== null && val > i) {
                                                        return <>
                                                            <DoneIcon />
                                                        </>
                                                    } else {
                                                        return <></>
                                                    }
                                                })
                                            }
                                            {/* {
                                                animatedCount.derive(val => {
                                                    if (props.message.val.data.status === ProgressStatuses.INPROGRESS && currentWorkId.val === i) {
                                                        return <>
                                                            <p class={css.Name}>{TranslationStore.t(`inprogress.${stage_name}`)}</p>
                                                        </>
                                                    } else {
                                                        console.warn("Animated Count = " + animatedCount.val);
                                                        return <>
                                                            <p class={`${css.Name} ${animatedCount.val >= i ? css.Completed : ""}`}>{TranslationStore.t(stage_name)}</p>
                                                        </>
                                                    }
                                                })
                                            } */}
                                            <p class={`${css.Name}`}>{TranslationStore.t(stage_name)}</p>
                                            <div class={css.Status}>
                                                {
                                                    currentWorkId.derive(val => {
                                                        // TODO: Fix PENDING nad INPROGRESS items handling.
                                                        // INPROGRESS triggers instantly after COMPLETE message is received.
                                                        // It's wrong behavior

                                                        if (val !== null && val > i) {
                                                            return <p>Completed</p>;
                                                        } else if (val === i) {
                                                                return <p>In Progress</p>;
                                                        } else if (val !== null && val < i) {
                                                            return <p class={css.Pending}>Pending</p>;
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
                        }
                    }, [props.progressItems])
                }
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