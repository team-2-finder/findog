import React, { useEffect, useState, useLocation } from "react";
import axios from "axios";
import { Header, SimilarityCard, MHeader, MBottomNavBar } from "../components";
import { bg2 } from "../images";
import styled from "styled-components";
import DetailModal from "../components/DetailModal";

const Similarity = () => {
  // const location = useLocation();
  // const arr = location.arr;
  const isMobile = window.innerWidth <= 393;
  const arr = [
    [
      {
        desertionNo: "448547202300713",
        filename:
          "http://www.animal.go.kr/files/shelter/2023/05/20230629140624.jpg",
        imagePath: "./images/448547202300713.jpg",
        happenDt: "20230629",
        kindCd: "[개] 진도견",
        colorCd: "흰색",
        age: "2014(년생)",
        weight: "14(Kg)",
        sexCd: "M",
        neuterYn: "N",
        careNm: "거창유기동물보호센터",
        careTel: "055-945-6500",
        careAddr: "경상남도 거창군 남상면 수남로 1934-12  ",
        chargeNm: "거창군",
        officetel: "055-940-8288",
        noticeComment: null,
      },
      0.33293691072407827,
    ],
    [
      {
        desertionNo: "448567202300905",
        filename:
          "http://www.animal.go.kr/files/shelter/2023/05/202306291106887.jpg",
        imagePath: "./images/448567202300905.jpg",
        happenDt: "20230629",
        kindCd: "[개] 믹스견",
        colorCd: "갈색+검정+흰색",
        age: "2023(년생)",
        weight: "3.8(Kg)",
        sexCd: "F",
        neuterYn: "N",
        careNm: "창원유기동물보호소",
        careTel: "055-225-5701",
        careAddr:
          "경상남도 창원시 의창구 창이대로 71 (명서동, 창원시농업기술센터) 축산과",
        chargeNm: "창원시의창성산구",
        officetel: "055-225-5701",
        noticeComment: null,
      },
      0.2730709475747689,
    ],
  ];

  return (
    <>
      {isMobile ? <MHeader /> : <Header />}
      <S.Container>
        <S.HeaderBox>
          사진과 유사한
          <br />
          강아지들을 찾아봤어요.
        </S.HeaderBox>
        <S.AnimalContainer>
          {arr.map((res, i) => (
            <SimilarityCard
              key={i}
              date={res[0].happenDt}
              kindCd={res[0].kindCd}
              sexCd={res[0].sexCd}
              neuterYn={res[0].neuterYn}
              imgUrl={res[0].filename}
              careNm={res[0].careNm}
              careTel={res[0].careTel}
              weight={res[0].weight}
              similar={Math.ceil(res[1] * 100)}
            />
          ))}
        </S.AnimalContainer>
      </S.Container>
      {isMobile && <MBottomNavBar />}
    </>
  );
};
const S = {
  Container: styled.div`
    padding-inline: 80px;
    @media screen and (max-width: 393px) {
      padding-inline: 24px;
    }
    background-image: url(${bg2});
    background-repeat: no-repeat;
    background-size: cover;
    background-attachment: fixed;
  `,
  HeaderBox: styled.div`
    font-size: 48px;
    padding-top: 12px;
    margin-block: 12px;
    font-weight: bold;
    @media screen and (max-width: 393px) {
      margin-block: 16px;
      font-size: 32px;
    }
  `,
  AnimalContainer: styled.div`
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    @media screen and (max-width: 393px) {
      grid-template-columns: 1fr;
    }
  `,
};

export default Similarity;
