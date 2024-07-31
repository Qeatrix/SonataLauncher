import { ref } from 'hywer';
import Api from '@/data/api';
import Store from '@/data/store';
import { VersionsManifest, Version, RequestInstance, ProgressTargetsList, ProgressMessage, ProgressStatuses, ProgressMessageFinish } from '@/data/types';

import { SelectionArea, SelectionItem } from '@/ui/components/selectionArea/selectionArea';
import { ContentStack, FlexBox, Window, WindowControls } from '@/ui/components/window/window';
import { ProgressDisplay } from '@/ui/widgets/progressDisplay/ProgressDisplay';
import Input from '@/ui/components/input/input';
import { For } from 'hywer/x/html';
import Button from '@/ui/components/buttons/buttons';


const Instances = () => {
    const rootPath = ref("");
    const versionsManifest = ref<VersionsManifest>({
        latest: {
            release: '',
            snapshot: ''
        },
        versions: []
    });

    const selectedVersion = ref<Version>({
        complianceLevel: 0,
        id: '',
        releaseTime: '',
        sha1: '',
        time: '',
        type: '',
        url: ''
    });

    const selectedVersionNumber = ref<number>(0);
    const contentStackIndex = ref<number>(0);
    const selectedVersionId = ref<string>("");
    const selectedVersionUrl = ref<string>("");
    const selectedLoaderNumber = ref(0);
    const progressItems = ref<ProgressTargetsList>({
        message_id: 'uninitialized',
        message_type: '',
        timestamp: '',
        ids_list: []
    });

    const progressUpdatedOnce = ref<boolean>(false);
    const progressMessage = ref<ProgressMessage>({
        message_id: "uninitialized",
        message_type: "",
        timestamp: "",
        data: {
            stage: ProgressStatuses.PENDING,
            determinable: false,
            progress: 0,
            max: 0,
            status: ProgressStatuses.PENDING,
            target_type: "FILE",
            target: {
                status: "",
                name: "",
                size_bytes: 0
            }
        }
    });

    const Loaders = [
            {
                "name": "Vanilla",
            },
            {
                "name": "Fabric",
            }
    ];

    const ws = new WebSocket('ws://127.0.0.1:8080/ws/instance/create');

    ws.onopen = () => {
        console.log('WebSocket connection established');
    };

    ws.onmessage = (event) => {
        const msg = JSON.parse(event.data);
        console.log(msg);

        if (msg.message_type === "PROGRESS_TARGETS_LIST") {
            progressItems.val = msg as ProgressTargetsList;
        } else if (msg.data) {
            progressMessage.val = msg as ProgressMessage;
            // console.log(progressMessage.val);
        } else if (msg.message_type === "PROGRESS_FINISH") {
            // progressMessage.val = msg as ProgressMessageFinish;
        }
    };


    setTimeout(() => {
        versionsManifest.val = Store.getGlobalManifestData() as unknown as VersionsManifest;
    }, 0)


    const createLauncherRoot = async () => {
        Api.createLauncherRoot(rootPath.val)
            .then(json => { console.log(json) })
            .catch(err => { console.log(err) })
    }

    const getVersionsManifest = async () => {
        Api.getVersionsManifest()
            .then(json => {
                Store.setGlobalManifestData(json);
                versionsManifest.val = json as unknown as VersionsManifest;
            })
            .catch(err => { console.log(err) })

        console.log(versionsManifest.val);
    }

    const requestVersionDownload = async () => {
        console.log(selectedVersion.val.url);

        const info = new Map<string, string>();
        info.set("${auth_player_name}", "Melicta");
        info.set("${version_name}", selectedVersion.val.id);
        info.set("${version_type}", selectedVersion.val.type);
        info.set("${user_type}", "legacy");
        info.set("${auth_uuid}", "99b3e9029022309dae725bb19e275ecb");
        info.set("${auth_access_token}", "[asdasd]");

        let infoObject: Record<string, string> = {};
        info.forEach((value, key) => {
            infoObject[key] = value;
        });

        // Api.requestVersionDownload("asd", selectedVersion.val.url, infoObject)
        //     .then(json => { console.log(json) })
        //     .catch(err => { console.log(err) })

        let body = JSON.stringify({
            name: "asd",
            url: selectedVersionUrl.val,
            info: infoObject
        })

        ws.send(body);
    }

    const handleVersionChange = async (id: string, url: string) => {
        selectedVersionId.val = id;
        selectedVersionUrl.val = url;
        console.log(selectedVersionId.val);
    }

    const handleLoaderChange = (key: number) => {
        selectedLoaderNumber.val = key;
    }

    const previousWindow = () => {
        contentStackIndex.val--;
    }

    const nextWindow = () => {
        requestVersionDownload();
        contentStackIndex.val++;
    }

    const tabSelection = ref(0);

    return (
        <>
            <input type="text" id="path" onInput={(e: { target: { value: string; }; }) => rootPath.val = e.target.value}></input>
            <button onClick={createLauncherRoot}>Create launcher root</button>
            <br /><br />
            <button onClick={getVersionsManifest}>Get Versions</button>
            <br /><br />
            <input type="text" id="instance-name" />
            {/* <div class={css.selectionArea}>
                {versionsManifest.derive(val => val?.versions.map((version, key) => (
                    <button
                        class={selectedVersionNumber.derive(val => `${css.button} ${val === key ? css.selectedButton : ''}`)}
                        key={key}
                        onClick={(e: PointerEvent) => handleVersionChange(version, key)}
                    >
                        {version.id}
                    </button>
                )))}
            </div> */}
            <button onClick={requestVersionDownload}>Continue</button>
            <Window name="Instance Creation">
                <ContentStack showIndex={contentStackIndex}>
                    <div>
                        <FlexBox>
                            <Input id="name" name="Name" />
                            <>
                                <div></div>
                            </>
                        </FlexBox>
                        <FlexBox>
                            <Input id="tags" name="Tags" />
                            <>
                                <div></div>
                            </>
                        </FlexBox>
                        <FlexBox>
                            <SelectionArea selectedValue={selectedVersionId} onValueChange={handleVersionChange} name="Versions" searchBar={true}>
                                {versionsManifest.derive(val => {
                                    return <For in={val.versions}>
                                        {(item, i) => {
                                            return <SelectionItem
                                                name={item.id}
                                                onClick={() => handleVersionChange(item.id, item.url)}
                                                selected={selectedVersionId.val == item.id}
                                            />
                                        }}
                                    </For>
                                })}
                            </SelectionArea>
                            <SelectionArea selectedValue={selectedVersionId} onValueChange={handleVersionChange} name="Loader">
                                <For in={Loaders}>
                                    {(loader, i) => {
                                        return <SelectionItem
                                            name={loader.name}
                                            onClick={() => handleLoaderChange(i)}
                                            selected={selectedLoaderNumber.val == i}
                                        />
                                    }}
                                </For>
                            </SelectionArea>
                        </FlexBox>
                    </div>
                    <div>
                        {/* <ProgressDisplay>
                            <div>
                                {derive(([progressItems, progressMessage]) => {
                                    return <For in={progressItems.val.ids_list}>
                                        {(stage) => {
                                            if (stage) {
                                                return <ProgressItem name_id={stage} message={progressMessage} />
                                            }
                                        }}
                                    </For>
                                }, [progressItems, progressMessage])}
                            </div>
                        </ProgressDisplay> */}
                        <ProgressDisplay progressItems={progressItems} message={progressMessage} />
                    </div>
                    <div>
                        <FlexBox>
                            <Input id="asd123" name="Name" />
                            <>
                                <div></div>
                            </>
                        </FlexBox>
                        <FlexBox>
                            <Input id="dkfjgh" name="Tags" />
                            <>
                                <div></div>
                            </>
                        </FlexBox>
                    </div>
                </ContentStack>
                <WindowControls>
                    <Button text="Cancel" onClick={previousWindow} />
                    <Button text="Continue" primary={true} onClick={nextWindow} />
                </WindowControls>
            </Window>
        </>
    );
};

export default Instances;
