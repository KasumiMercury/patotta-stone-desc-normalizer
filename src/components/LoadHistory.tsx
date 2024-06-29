import type {LoadHistory} from "../App";

interface LoadHistoryProps {
    histories: LoadHistory[];
}

export default function LoadHistoryList(props: LoadHistoryProps){
    return (
        <div>
        <h1>LoadHistory</h1>
        <ul>
            {props.histories.map((history:LoadHistory) => (
                <li key={history.id}>
                    <p>File path: {history.path}</p>
                    <p>Loaded at: {history.loaded_at}</p>
                </li>
            ))}
        </ul>
        </div>
    );
}
