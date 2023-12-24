import { Link } from "react-router-dom";

export default function Navigation() {
    return (
        <div className="flex gap-5 p-5">
            <Link className="border border-black p-2 rounded" to={"/office"}>Office</Link>
            <Link className="border border-black p-2 rounded" to={"/office/items"}>Items</Link>
            <Link className="border border-black p-2 rounded" to={"/office/items/joe"}>Joe</Link>
        </div>
    )
}