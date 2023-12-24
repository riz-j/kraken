export default function SiteHeader() {
    return (
        <>
            <div>
                <div className="flex justify-between items-center bg-neutral max-h-16 py-5 px-5 lg:px-10">
                    {/* <!-- Logo --> */}
                    <div className="flex items-center h-16 py-5">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 925 200" fill="none" className="hidden sm:block h-4 md:h-7">
                            <path fill="currentColor" className="text-neutral-content h-full" d="m204.224 79.164-.28-.746-28.267-73.733a7.334 7.334 0 0 0-2.906-3.507 7.468 7.468 0 0 0-11.213 4.267l-19.08 58.413H65.212L46.132 5.445a7.481 7.481 0 0 0-11.213-4.267 7.333 7.333 0 0 0-2.907 3.507L3.759 78.43l-.294.733a52.48 52.48 0 0 0 17.414 60.653l.106.08.24.187 43 32.226 21.333 16.12 12.96 9.8a8.757 8.757 0 0 0 10.56 0l12.96-9.8 21.333-16.12 43.306-32.413.12-.093a52.48 52.48 0 0 0 17.427-60.64ZM373.449 75.35h30.373c-5.067-32.292-33.026-54.785-69.413-54.785-43.053 0-75.359 31.652-75.359 84.665 0 52.067 30.853 84.36 76.239 84.36 40.733 0 69.893-26.2 69.893-68.613v-19.76h-66.759v23.293h37.919c-.48 23.467-16.146 38.32-40.893 38.32-27.546 0-46.426-20.64-46.426-57.92 0-37.04 19.2-57.6 45.786-57.6 19.84 0 33.413 10.6 38.64 28.04ZM425.261 187.336h29.08V64.004h-29.08v123.332Zm14.667-142.932c9.24 0 16.787-7.08 16.787-15.746 0-8.667-7.547-15.827-16.787-15.827-9.24 0-16.866 7.067-16.866 15.827s7.493 15.746 16.813 15.746h.053ZM541.421 64.004h-24.346V34.378h-29.08v29.626h-17.507v22.493h17.507v68.613c-.16 23.213 16.72 34.667 38.559 33.987a59.05 59.05 0 0 0 17.12-2.894l-4.906-22.746a35.4 35.4 0 0 1-8.507 1.133c-7.32 0-13.186-2.573-13.186-14.306V86.431h24.346V64.004ZM563.928 187.337h102.266V162.35h-72.453V22.805h-29.813v164.532ZM721.9 189.83c19.36 0 30.92-9.08 36.226-19.44h.96v16.947h28v-82.586c0-32.613-26.586-42.413-50.133-42.413-25.946 0-45.866 11.56-52.293 34.053l27.16 3.867c2.88-8.44 11.08-15.667 25.333-15.667 13.507 0 20.893 6.907 20.893 19.04v.48c0 8.347-8.76 8.76-30.533 11.08-23.933 2.573-46.826 9.72-46.826 37.52-.08 24.266 17.666 37.119 41.213 37.119Zm9.56-21.333c-12.133 0-20.813-5.547-20.813-16.226 0-11.174 9.72-15.827 22.666-17.68 7.64-1.04 22.907-2.974 26.667-6.014v14.534c.08 13.693-11.014 25.346-28.52 25.346v.04ZM808.499 187.337h28.6v-19.44h1.68c4.586 9 14.146 21.613 35.359 21.613 29.08 0 50.853-23.066 50.853-63.706 0-41.133-22.413-63.466-50.933-63.466-21.773 0-30.853 13.093-35.279 22.013h-1.227V22.805h-29.053v164.532Zm28.52-61.693c0-24 10.28-39.453 29-39.453 19.359 0 29.333 16.466 29.333 39.453 0 22.986-10.12 40-29.333 40-18.56 0-29-16.067-29-40Z">
                            </path>
                        </svg>
                    </div>

                    <div className="flex gap-4 items-center h-full">
                        {/* <!-- Search Bar Button --> */}
                        <div 
                            v-if="!searchBarFocus" 
                            className="btn flex justify-between bg-base-100 w-72"
                        >
                            <div className="opacity-60">Search</div>
                            <div className="flex items-center gap-1 opacity-60">
                                <div className="kbd">Ctrl</div>
                                <div className="text-lg">+</div>
                                <div className="kbd">K</div>
                            </div>
                        </div>

                        {/* <!-- Search Bar Input --> */}
                        {/* <div v-if="searchBarFocus" className="blur-backdrop"></div>
                        <div v-if="searchBarFocus" className="flex flex-col text-black shadow rounded-lg z-50">
                            <input  
                                className="input w-[40rem] bg-white rounded-lg
                                focus:outline-none focus:ring-0 focus:border-transparent" 
                                v-model="searchQuery"
                            />
                            <div className="flex flex-col w-full mt-[-6px] border-t border-black-200 rounded-b-lg bg-white">
                                <a 
                                    v-for="result in searchResults" 
                                    className="flex items-center w-full px-5 py-3 gap-3
                                    hover:bg-primary cursor-pointer"
                                >
                                    <img src="/images/icons/news.png" className="h-6" />
                                    <span className="font-bold">{ "Aweosome Stuff" }</span>
                                </a>
                                <div v-if="searchResults.length > 0" className="h-3"></div>
                            </div>
                        </div> */}
                            
                        {/* <!-- Theme --> */}
                        <div className="dropdown dropdown-end">
                            <label className="btn">Theme</label>
                            <ul className="dropdown-content join join-vertical z-[1] menu shadow bg-base-100 rounded-box">
                                <li v-for="theme in themes" className="btn join-item my-1">{ "Joe" }</li>
                            </ul>
                        </div>

                    </div>
                </div>
            </div>
        </>
    )
}