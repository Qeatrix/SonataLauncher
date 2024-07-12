function page() {
    const ws = new WebSocket('ws://127.0.0.1:8080/ws/instance/create');

    ws.onopen = () => {
        console.log('WebSocket connection established');
    };

    ws.onmessage = (event) => {
        console.log(`Received message: ${event.data}`);
        console.log(event);
    };

    const handleConnetion = () => {
        ws.send("asd");
    }

    return (
        <>
            <button onClick={handleConnetion}>Test Socket</button>
        </>
    )
}

export default page;