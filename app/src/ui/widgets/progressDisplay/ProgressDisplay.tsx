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
import { constructElementIconId, constructElementId, constructElementNameId, getLastProcessedElement, getWorkNames } from "./element";
import { startElapseTimer, startWorknameBlinkTimer, stopElapseTimer, stopWorknameBlinkTimer } from "./timers";


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
    const currentWorkId = ref<number>(-1);
    const isCurrentWorkIdINPORGRESS = ref<boolean>(false);
    const currentWorkProgressPercentage = ref<number>(0);
    const taskElapsedTime = ref<string>("0.0s");

    let pendingClassRemoved = false;
    let worknamesBlinked = false;
    let earlyTweaksDone = false;
    let timerInterval: any | null = null;
    let blinkInterval: any | null = null;


    props.message.sub = (val) => {
        // Get the id of the stage with last processed element
        currentWorkId.val = props.progressItems.val.ids_list.indexOf(val.data.stage);
        updateWorkName(val);

        if (val.data.status === ProgressStatuses.INPROGRESS) {
            const lastProcessedElement = getLastProcessedElement(componentId, currentWorkId);
            isCurrentWorkIdINPORGRESS.val = true;
            currentWorkProgressPercentage.val = val.data.progress / val.data.max * 100;

            if (timerInterval === null) {
                timerInterval = startElapseTimer(taskElapsedTime);
            }

            if (blinkInterval === null) {
                blinkInterval = startWorknameBlinkTimer(componentId, currentWorkId, css, worknamesBlinked);
            }

            // Highlight inprogress operation by increasing margins
            if (earlyTweaksDone === false) {
                gsap.to(lastProcessedElement, {
                    marginTop: '10px',
                    marginBottom: '10px',
                    ease: 'power1.InOut',
                    duration: 0.35,
                })

                earlyTweaksDone = true;
            }

            // const workNames = getWorkNames(componentId, currentWorkId.val);
            // if (pendingClassRemoved === false && workNames) {
            //     gsap.to(workNames[0], {
            //         beforeStart: () => {
            //             workNames[0].classList.remove(css.Pending);
            //         },
            //         opacity: 1,
            //         ease: 'linear',
            //         duration: 1,
            //     })

            //     pendingClassRemoved = true;
            // }
        }

        if (val.data.status === ProgressStatuses.COMPLETED) {
            isCurrentWorkIdINPORGRESS.val = false;

            if (timerInterval) {
                stopElapseTimer(timerInterval, taskElapsedTime);
                timerInterval = null;
            }

            if (blinkInterval) {
                stopWorknameBlinkTimer(blinkInterval, worknamesBlinked);
                blinkInterval = null;
            }

            const lastProcessedElement = getLastProcessedElement(componentId, currentWorkId);
            const doneIcon = document.getElementById(`ProgressItemIcon-${componentId}-${currentWorkId.val}`);

            if (lastProcessedElement && doneIcon) {
                setTimeout(() => {
                    gsap.to(lastProcessedElement, {
                        marginTop: 0,
                        marginBottom: 0,
                        ease: 'power1.InOut',
                        duration: 0.35,
                    })

                    const workNames = getWorkNames(componentId, currentWorkId.val);
                    if (workNames) {
                        gsap.to(workNames[0], {
                            opacity: 1,
                            left: absposCompletedWorknameOffset,
                            ease: 'power1.InOut',
                            duration: 0.35,
                            onComplete: () => {
                                console.log("Offset changed");
                            }
                        })

                        gsap.to(workNames[1], {
                            opacity: 0,
                            left: absposCompletedWorknameOffset,
                            ease: 'power1.InOut',
                            duration: 0.35,
                        })

                        gsap.to(workNames, {
                            fontVariationSettings: "'wght' " + 500,
                            fontSize: "1rem",
                            left: absposCompletedWorknameOffset,
                            ease: 'power1.InOut',
                            duration: 0.35,
                        })

                        // workNames[0].classList.remove(css.Pending);
                    }

                    gsap.to(doneIcon, {
                        opacity: 1,
                        scale: 1,
                        ease: 'back.out(1.7)',
                        duration: 0.35,
                        delay: 0.2,
                    })

                    // We need to update currentWorkId only after playing all the animations.
                    // Otherwise the animations will be played after value increasing the value.
                    // This will cause an offset, which we need to avoid.
                    if (currentWorkId.val !== null) {
                        currentWorkId.val++;
                    }
                });
            }
        }
        pendingClassRemoved = false;
        earlyTweaksDone = false;
    }

    isCurrentWorkIdINPORGRESS.sub = (val) => {
        const lastProcessedElement = getLastProcessedElement(componentId, currentWorkId);

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

    const updateWorkName = (val: ProgressMessage) => {
        const workNames = getWorkNames(componentId, currentWorkId.val);

        if (workNames) {
            if (val.data.status === ProgressStatuses.INPROGRESS) {
                workNames[0].innerText = TranslationStore.t(`inprogress.${val.data.stage}`);
            } else {
                workNames[0].innerText = TranslationStore.t(`${val.data.stage}`);
            }
        }
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
                                        } id={constructElementId(componentId, i)}>
                                            <div class={css.Container}>
                                                <DoneIcon style="opacity:0" id={constructElementIconId(componentId, i)} />
                                                <div class={css.Container}>
                                                    <div class={css.NameContainer}>
                                                        <p class={`${css.Name}`} id={constructElementNameId(componentId, i, 0)}>{TranslationStore.t(stage_name)}</p>
                                                        <p class={`${css.Name}`} style="opacity:0" id={constructElementNameId(componentId, i, 1)}>{
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
                                                                } else if (currentWorkId.val < i ||
                                                                        (currentWorkId.val === i && isCurrentWorkIdINPORGRESS.val === false)) {
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
