import React, { useEffect, useState } from "react";
import axios from "axios";
import { useLocation, useNavigate } from "react-router-dom";
import {
  Header,
  SimilarityCard,
  MHeader,
  MBottomNavBar,
  MainColor,
} from "../components";
import { bg2 } from "../images";
import styled from "styled-components";
import { noItem, noItemText } from "../images";

const Similarity = () => {
  const location = useLocation();
  const navigate = useNavigate();
  const arr = location.state.arr;
  const isMobile = window.innerWidth <= 393;

  return (
    <>
      {isMobile ? <MHeader /> : <Header />}
      <S.Container>
        <S.HeaderBox>
          사진과 유사한
          <br />
          강아지들을 찾아봤어요.
        </S.HeaderBox>
        {arr.length !== 0 ? (
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
        ) : (
          <div style={{ width: "100%" }}>
            <S.NoItem src={noItem} />
            <S.NoItemText src={noItemText} />
            <S.BackButton
              onClick={() => {
                navigate("/");
              }}
            >
              돌아가기
            </S.BackButton>
          </div>
        )}
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
  NoItem: styled.img`
    position: absolute;
    top: 30%;
    left: 35%;
    width: 30%;
    height: 30%;
  `,
  NoItemText: styled.img`
    position: absolute;
    top: 45%;
    left: 35%;
    width: 30%;
    height: 30%;
  `,
  BackButton: styled.div`
    width: 500px;
    height: 50px;
    position: absolute;
    cursor: pointer;
    top: 65%;
    left: 34%;
    color: white;
    text-align: center;
    padding-top: 15px;
    font-size: 24px;
    border-radius: 16px;
    background-color: ${MainColor};
  `,
};

export default Similarity;
