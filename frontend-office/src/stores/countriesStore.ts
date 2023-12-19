import { create } from 'zustand';
import { devtools } from 'zustand/middleware';
import { CountrySummarizedSchema } from '@kraken_bindings/CountrySummarizedSchema';

interface CountriesState {
    countries: CountrySummarizedSchema[];
    populateCountries: () => Promise<void>;
}

const useCountriesStore = create<CountriesState>()(
  devtools(
    (set) => ({
      countries: [],
      populateCountries: async () => {
          const response = await fetch('http://localhost:2900/countries');
          const result = await response.json();
          set({ countries: result });
      },
    }),
    { name: "CountriesStore" }
  )
);

export default useCountriesStore;
