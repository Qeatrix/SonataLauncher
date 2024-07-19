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


// Values for animation
const iconGap = 5;
const iconSize = 20;
const defaultIconSize = 20;

const absposMeasuredIconSize = 32;
const absposElementOffset = absposMeasuredIconSize / defaultIconSize;

const absposCompletedWorknameOffset = iconGap * absposElementOffset + iconSize * absposElementOffset;

const progressbarTopOffset = 10;


interface IProgressDisplay {
    children?: JSX.Element | JSX.Element[],
    progressItems: Reactive<ProgressTargetsList>,
    message: Reactive<ProgressMessage>
}

export function ProgressDisplay(props: IProgressDisplay) {
    const componentId = Store.makeId(6);
    const currentWorkId = ref<number | null>(null);
    const completedElements = ref<HTMLElement[]>([]);
    const isCurrentWorkIdINPORGRESS = ref<boolean>(false);
    const currentWorkProgressPercentage = ref<number>(0);
    const taskElapsedTime = ref<string>("0.0s");

    let classesGived = false;
    let worknamesBlinked = false;
    let earlyTweaksDone = false;
    let timerInterval: any | null = null;
    let blinkInterval: any | null = null;

    props.message.sub = (val) => {
        // Get the id of the stage with last processed element
        currentWorkId.val = props.progressItems.val.ids_list.indexOf(val.data.stage);

        updateWorkName(val);

        if (val.data.status === ProgressStatuses.INPROGRESS) {
            const lastProcessedElement = getLastProcessedElement();
            isCurrentWorkIdINPORGRESS.val = true;
            currentWorkProgressPercentage.val = val.data.progress / val.data.max * 100;

            if (timerInterval === null) {
                timerInterval = startElapseTimer();
            }

            if (blinkInterval === null) {
                blinkInterval = startWorknameBlinkTimer();
            }

            if (earlyTweaksDone === false) {
                console.error(lastProcessedElement);
                gsap.to(lastProcessedElement, {
                    marginTop: '10px',
                    marginBottom: '10px',
                    ease: 'power1.InOut',
                    duration: 0.35,
                })

                earlyTweaksDone = true;
            }

/*             if (classesGived === false) {
                const elements = lastProcessedElement?.getElementsByClassName(css.Name);

                if (elements) {
                    for (let i = 0; i < elements.length; i++) {
                        elements[i].classList.add(css.Blinking);
                    }

                    classesGived = true;
                }
            } */
        }

        if (val.data.status === ProgressStatuses.COMPLETED) {
            isCurrentWorkIdINPORGRESS.val = false;

            if (timerInterval) {
                stopElapseTimer(timerInterval);
                timerInterval = null;
            }

            if (blinkInterval) {
                stopWorknameBlinkTimer(blinkInterval);
                blinkInterval = null;
            }

            const lastProcessedElement = getLastProcessedElement();

            if (lastProcessedElement) {
                completedElements.val.push(lastProcessedElement);

                const workName = completedElements.val[currentWorkId.val].getElementsByClassName(css.Name);
                const doneIcon = completedElements.val[currentWorkId.val].getElementsByTagName("svg");

                if (workName && doneIcon) {
/*                     for (let i = 0; i < workName.length; i++) {
                        workName[i].classList.remove(css.Blinking);
                    } */

                    setTimeout(() => {
                        gsap.to(lastProcessedElement, {
                            marginTop: 0,
                            marginBottom: 0,
                            ease: 'power1.InOut',
                            duration: 0.35,
                        })

                        gsap.to(workName[0], {
                            opacity: "1",
                            left: absposCompletedWorknameOffset,
                            ease: 'power1.InOut',
                            duration: 0.35,
                        })

                        gsap.to(workName[1], {
                            opacity: "0",
                            left: absposCompletedWorknameOffset,
                            ease: 'power1.InOut',
                            duration: 0.35,
                        })

                        gsap.to(workName, {
                            fontVariationSettings: "'wght' " + 500,
                            fontSize: "1rem",
                            left: absposCompletedWorknameOffset,
                            ease: 'power1.InOut',
                            duration: 0.35,
                        })

                        gsap.to(doneIcon, {
                            opacity: 1,
                            scale: 1,
                            ease: 'back.out(1.7)',
                            duration: 0.35,
                            delay: 0.2,
                        })
                    });
                }
            }

            currentWorkId.val++;
            classesGived = false;
            earlyTweaksDone = false;
        }
    }

    isCurrentWorkIdINPORGRESS.sub = (val) => {
        const lastProcessedElement = getLastProcessedElement();

        if (lastProcessedElement) {
            const progressElement = lastProcessedElement.getElementsByTagName("progress");
            gsap.to(progressElement, {
                opacity: val ? 1 : 0,
                marginTop: val ? `${progressbarTopOffset}px` : `0px`,
                ease: 'power1.inOut',
                duration: 0.1,
            })

            const workName = lastProcessedElement.getElementsByClassName(css.Name);
            if (workName) {
                gsap.to(workName, {
                    fontSize: "1.25rem",
                    fontVariationSettings: "'wght' " + 500,
                    ease: 'power1.InOut',
                    duration: 0.35,
                })
            }
        }
    }

    currentWorkId.sub = (val) => {
        const lastProcessedElement = getLastProcessedElement();

        if (lastProcessedElement) {
            gsap.to(lastProcessedElement, {
                opacity: 1,
                ease: 'power1.inOut',
                duration: 0.1,
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

    const startElapseTimer = () => {
        const timerStartTime = new Date();

        const timerInterval = setInterval(() => {
            const elapsedSeconds = new Date().getTime() - timerStartTime.getTime();
            const seconds = (elapsedSeconds / 1000).toFixed(1);
            taskElapsedTime.val = `${seconds}s`;
        }, 100)

        return timerInterval;
    }

    const stopElapseTimer = (timerInterval: number) => {
        clearInterval(timerInterval);
        taskElapsedTime.val = "0.0s";
    }

    const startWorknameBlinkTimer = () => {
        const lastProcessedElement = getLastProcessedElement();
        const workName = lastProcessedElement?.getElementsByClassName(css.Name);

        const transitionType = "power4.Out"
        const transitionDuration = 1;

        return setInterval(() => {
            if (workName) {
                gsap.to(workName[0], {
                    opacity: worknamesBlinked ? 1 : 0,
                    ease: transitionType,
                    duration: transitionDuration,
                })

                gsap.to(workName[1], {
                    opacity: worknamesBlinked ? 0 : 1,
                    ease: transitionType,
                    duration: transitionDuration,
                })

                if (worknamesBlinked) {
                    worknamesBlinked = false;
                } else {
                    worknamesBlinked = true;
                }
            } else {
                console.warn("Workname not found");
            }
        }, 4000)
    }

    const stopWorknameBlinkTimer = (timerInterval: number) => {
        clearInterval(timerInterval);
        worknamesBlinked = false;
    }

    return (
        <>
            <div class={css.ProgressDisplay}>
                {
                    derive(([progressItems]) => {
                        if (progressItems.val.ids_list.length === 0) {
                            return <>
                                <p>Nothing to show</p>
                            </>
                        } else {
                            return <For in={progressItems.val.ids_list}>
                                {(stage_name, i) => {
                                    console.warn("PROGRESS ITEM: " + stage_name);
                                    return <>
                                        <div className={
                                            currentWorkId.derive(val => val !== null && val < i ? `${css.Item} ${css.Pending}` : `${css.Item}`)
                                        } id={`ProgressItem-${componentId}-${i}`}>
                                            <div class={css.Container}>
                                                <DoneIcon style="opacity:0" />
                                                <div class={css.Container}>
                                                    <div class={css.NameContainer}>
                                                        <p class={`${css.Name}`}>{TranslationStore.t(stage_name)}</p>
                                                        <p class={`${css.Name}`} style="opacity:0">{
                                                            props.message.derive(val => {
                                                                // TODO: Second <p> may still be shown after process completion
                                                                if (val.data.progress !== null && val.data.max > 0) {
                                                                    return `Downloading · ${val.data.progress}/${val.data.max}`;
                                                                } else {
                                                                    return TranslationStore.t(`inprocess.${stage_name}`);
                                                                }
                                                            })
                                                            }</p>
                                                    </div>
                                                    <div class={css.Status}>
                                                        {
                                                            derive(([currentWorkId]) => {
                                                                if (currentWorkId.val !== null && currentWorkId.val > i) {
                                                                    return <p>Completed</p>;
                                                                } else if (currentWorkId.val === i && isCurrentWorkIdINPORGRESS.val === true) {
                                                                        return <p>
                                                                            {
                                                                                taskElapsedTime.derive(val => {
                                                                                    return val;
                                                                                })
                                                                            }
                                                                        </p>;
                                                                } else if (currentWorkId.val !== null &&
                                                                        (currentWorkId.val < i ||
                                                                        (currentWorkId.val === i && isCurrentWorkIdINPORGRESS.val === false))) {
                                                                    return <p class={css.Pending}>Pending</p>;
                                                                } else {
                                                                    return <p>Unknown</p>;
                                                                }
                                                            }, [currentWorkId, isCurrentWorkIdINPORGRESS])
                                                        }
                                                    </div>
                                                </div>
                                            </div>
                                            <progress value={currentWorkProgressPercentage.derive(val => val)} min="0" max="100" style="opacity:0"></progress>
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