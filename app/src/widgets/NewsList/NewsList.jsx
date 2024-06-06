import NewPreview from './assets/new-preview-example.png';
import './NewsList.css';

function NewsList() {
    const data = [
        {
            "tag": "minecraft news",
            "name": "new on realms: affordable housing",
            "imglink": "asd",
            "descr": "if there is one thing i wish i could give to the next generation that’s coming up in this increasingly expensive world, it would be better housing opportunities. barring that, maybe some new minecraft maps from our community would suffice? for those of us that are not castle - accommodated (if you are, hook me up with your supplier), it’s rare that you have an optimal living situation – these maps understand you!",
            "date": "april 29, 2022"
        },
        {
            "tag": "minecraft news",
            "name": "new on realms: affordable housing",
            "imglink": "asd",
            "descr": "if there is one thing i wish i could give to the next generation that’s coming up in this increasingly expensive world, it would be better housing opportunities. barring that, maybe some new minecraft maps from our community would suffice? for those of us that are not castle - accommodated (if you are, hook me up with your supplier), it’s rare that you have an optimal living situation – these maps understand you!",
            "date": "april 29, 2022"
        }
    ]

    return (
        <>
            <div className="feed">
                {data.map((news, key) => (
                    <div key={key} className="new">
                        <div className="new-header">
                            <div className="new-tag">{news.tag}</div>
                            <div className="new-name">{news.name}</div>
                        </div>
                        {
                            news.imglink != "" ?
                            <img src={NewPreview} alt="" />
                            :
                            <div className="">img is null</div>
                        }
                        <div className="new-descr">{news.descr}</div>
                        <div className="new-footer">
                            <a href="https://mojang.com" target="_blank" className="new-open">Read More</a>
                            <p className="new-date">{news.date}</p>
                        </div>
                    </div>
                ))}
            </div>
        </>
    ) 
}

export default NewsList;
