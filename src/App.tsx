import { open } from "@tauri-apps/api/dialog";
import { useEffect, useState } from "react";
import "./App.css";
import { invoke } from "@tauri-apps/api/tauri";
import * as dayjs from "dayjs";
import utc from "dayjs/plugin/utc";
import timezone from "dayjs/plugin/timezone";
import LoadHistoryList from "./components/LoadHistory.tsx";

// Add timezone support
dayjs.extend(utc);
dayjs.extend(timezone);

type ISODateString = string & { __brand: "ISODateString" };

function isISODateString(value: string): value is ISODateString {
	return dayjs(value).isValid();
}

function toISODateString(value: string): ISODateString {
	if (!isISODateString(value)) {
		throw new Error("Invalid ISO date string");
	}
	return value;
}

export interface LoadHistory {
	id: number;
	path: string;
	count: number;
	loaded_at: ISODateString;

	getLoadedAtJST(): dayjs.Dayjs;
}

interface ExistenceInfo {
	exists: boolean;
	count: number;
	last_loaded_at: ISODateString;
	histories: LoadHistory[];

	getLastLoadedAtJST(): dayjs.Dayjs;
}

class LoadHistoryImpl implements LoadHistory {
	constructor(
		public id: number,
		public path: string,
		public count: number,
		public loaded_at: ISODateString,
	) {}

	getLoadedAtJST(): dayjs.Dayjs {
		return dayjs(this.loaded_at).tz("Asia/Tokyo");
	}
}

class ExistenceInfoImpl implements ExistenceInfo {
	constructor(
		public exists: boolean,
		public count: number,
		public last_loaded_at: ISODateString,
		public histories: LoadHistory[],
	) {}

	getLastLoadedAtJST(): dayjs.Dayjs {
		return dayjs(this.last_loaded_at).tz("Asia/Tokyo");
	}
}

async function fetchExistenceInfo(): Promise<ExistenceInfo> {
	try {
		const result = await invoke("check_data_existence");
		const data = JSON.parse(result as string);
		return new ExistenceInfoImpl(
			data.exists,
			data.count,
			toISODateString(data.last_loaded_at),
			data.histories.map(
				(history: LoadHistory) =>
					new LoadHistoryImpl(
						history.id,
						history.path,
						history.count,
						toISODateString(history.loaded_at),
					),
			),
		);
	} catch (error) {
		console.error("Failed to fetch existence info", error);
		throw error;
	}
}

function App() {
	const [isLoaded, setIsLoaded] = useState(false);
	const [filePath, setFilePath] = useState("");
	const [openConfirmDialog, setOpenConfirmDialog] = useState(false);
	const [error, setError] = useState("");
	const [histories, setHistories] = useState<LoadHistory[]>([]);

	// Check the data has already existed in the sqlite database
	// If the data is already existed, load the data from the database
	// If the data is not existed, display button to open dialog of file selection
	useEffect(() => {
		(async () => {
			const existenceInfo = await fetchExistenceInfo();

			if (existenceInfo.exists) {
				setIsLoaded(true);
				setHistories(existenceInfo.histories);
			} else {
				setIsLoaded(false);
			}
		})();
	}, []);

	function openLoadDialog() {
		open({
			directory: false,
			multiple: false,
			// filters: [{ name: 'csv', extensions: ['csv'] }],
		}).then((res) => {
			// if res is null, the user closed the dialog without selecting a file
			if (res === null) {
				return;
			}
			// if res is an array, occurs error
			// can't select multiple files
			if (Array.isArray(res)) {
				console.error("Error: open dialog return an array");
				return;
			}
			setFilePath(res);
			setOpenConfirmDialog(true);
		});
	}

	async function loadCSV() {
		// load csv file
		await invoke("load_csv", { path: filePath });
	}

	function loadFile() {
		// load file
		loadCSV()
			.then(() => {
				setIsLoaded(true);
				setOpenConfirmDialog(false);
			})
			.catch((err) => {
				console.error(err);
				setError(err);
				setIsLoaded(false);
				setOpenConfirmDialog(false);
			});
	}

	return (
		<div>
			{isLoaded ? (
				<div>
					<LoadHistoryList histories={histories} />
				</div>
			) : (
				<div className="absolute inset-0 flex justify-center items-center">
					<div className="w-fit h-fit flex flex-col items-center">
						<h1 className="w-fit text-nowrap">Data is not loaded</h1>
						<button
							onClick={openLoadDialog}
							type={"button"}
							className="border-b-stone-400 border py-2 px-4 rounded-full my-2 bg-stone-800 shadow-md shadow-stone-900"
						>
							Select File
						</button>
					</div>
				</div>
			)}
			{/* the dialog to load the file */}
			{openConfirmDialog && (
				<div className="absolute inset-0 flex justify-center items-center bg-zinc-800/40">
					<div className="w-5/6 bg-zinc-800 shadow-md shadow-zinc-300/40 flex flex-col gap-y-3 px-3 rounded-lg border border-zinc-100 items-center">
						<p className="text-lg pt-4">Are you sure you want to load the file?</p>
						<p className="text-xs text-pretty text-center py-2">
							file path: {filePath}
						</p>
						<div className="w-full flex py-3 border-t">
							<button className="grow" type={"button"} onClick={() => setOpenConfirmDialog(false)}>No</button>
							<div className="w-1 grow-0 border-l mx-2" />
							<button className="grow" type={"button"} onClick={loadFile}>Yes</button>
						</div>
					</div>
				</div>
			)}
			{error && <p>Error: {error}</p>}
		</div>
	);
}

export default App;
