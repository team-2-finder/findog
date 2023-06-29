import Header from "./Header";
import MHeader from "./MHeader";
import Loading from "./Loading";
import { MainColor, DisableMain } from "./Colors";
import AnimalCard from "./AnimalCard";
import SimilarityCard from "./SimilarityCard";

import Label from "./Label";
import DetailModal from "./DetailModal";
import DetailModal2 from "./DetailModal2";
import MBottomNavBar from "./MBottomNavBar";
import * as serviceWorkerRegistration from './serviceWorkerRegistration';

serviceWorkerRegistration.register();

export {
  Header,
  MainColor,
  DisableMain,
  Loading,
  AnimalCard,
  SimilarityCard,
  MHeader,
  Label,
  DetailModal,
  DetailModal2,
  MBottomNavBar,
};
