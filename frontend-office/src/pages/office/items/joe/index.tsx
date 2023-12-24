import SiteHeader from "@/components/SiteHeader";
import { CityExtendedSchema } from "@kraken_bindings/CityExtendedSchema";
import { useEffect, useState } from "react";

export default function Joe() {
    const [city, setCity] = useState<CityExtendedSchema | undefined>();

    const fetchCity = async () => {
        const response = await fetch("http://localhost:2900/api/cities/2");
        const result = await response.json() as CityExtendedSchema;
        setCity(result);
    }

    useEffect(() => {
        fetchCity();
    }, []);

    return (
        <>
            {/* <Navigation /> */}
            <SiteHeader />
            <div className="py-5 px-5 lg:px-10">
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-8">
                    
                    {/* <!-- Enable Notifications Menu --> */}
                    <div className="flex flex-row items-center justify-start gap-4 card bg-base-200 p-5">
                        <div>
                            <input type="checkbox" className="toggle toggle-success" />
                        </div>
                        <div className="flex flex-col justify-center">
                            <h1 className=" font-bold">Enable Notifications</h1>
                            <h3 className="text-sm opacity-60">To get the latest updates</h3>
                        </div>
                    </div>

                    {/* <!-- Download Progress Menu --> */}
                    <div className="flex flex-col items-start justify-center gap-4 card bg-base-200 p-5">
                        <div className="flex flex-row justify-between items-center w-full">
                            <div className="flex flox-row gap-4">
                                <span className="loading loading-spinner"></span>
                                <h1 className="font-bold">Downloading</h1>
                            </div>
                            <div className="btn btn-sm text-sm opacity-60 hover:text-error">
                                Cancel
                            </div>
                        </div>
                        <progress className="progress progress-success" value="40" max="100"></progress>
                    </div>
                </div>
            </div>



            <div className="p-10">
                <p>Welcome to { city?.name } in the nation of { city?.country.name }</p>

                <p>{ city?.name } is located in the continent of { city?.country.continent }</p>
            </div>
        </>
    )
}